import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import org.openqa.selenium.Keys as Keys

Response = WS.sendRequest(findTestObject('004_J4U_Apioffers_automation/01_Target_Normal/03_j4u-Target Integrated'), FailureHandling.CONTINUE_ON_FAILURE)

offer1 = WS.getElementPropertyValue(Response, 'Offers[0].OfferID', FailureHandling.CONTINUE_ON_FAILURE)

offer2 = WS.getElementPropertyValue(Response, 'Offers[1].OfferID', FailureHandling.CONTINUE_ON_FAILURE)

offer3 = WS.getElementPropertyValue(Response, 'Offers[2].OfferID', FailureHandling.CONTINUE_ON_FAILURE)

def list = ['611164','611163','611155']

println(offer1)

println(offer2)

println(offer3)

assert offer1 in list

assert offer2 in list

assert offer3 in list

WS.comment('Normal ML Integrated offers flow is working fine for Targeted customer.')

