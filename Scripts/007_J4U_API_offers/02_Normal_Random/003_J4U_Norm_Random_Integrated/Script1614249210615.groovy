import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
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
import org.openqa.selenium.Keys as Keys

//GlobalVariable.j4u_Apioffer_random_msisdn = '123411111'
Response = WS.sendRequest(findTestObject('004_J4U_Apioffers_automation/02_Random Normal/j4u-Normal Random Integrated'), FailureHandling.CONTINUE_ON_FAILURE)

offer1 = WS.getElementPropertyValue(Response, 'Offers[0].OfferID', FailureHandling.CONTINUE_ON_FAILURE)

offer2 = WS.getElementPropertyValue(Response, 'Offers[1].OfferID', FailureHandling.CONTINUE_ON_FAILURE)

offer3 = WS.getElementPropertyValue(Response, 'Offers[2].OfferID', FailureHandling.CONTINUE_ON_FAILURE)

def list = ['611164', '611163', '611162', '611161', '611160', '611159', '611158', '611157', '611156', '611155', '611154', '611153', '611083', '611082', '611081', '611071', '611070', '611069', '611068', '611067', '611066', '611065', '611064', '611063', '611062', '611061', '611060', '611059', '611058', '611074', '611073', '611072', '611080', '611079', '611078', '611077', '611076', '611075']

println(offer1)

println(offer2)

println(offer3)

assert offer1 in list

assert offer2 in list

assert offer3 in list

WS.comment('Normal Random Integrated offers flow is working fine.')

