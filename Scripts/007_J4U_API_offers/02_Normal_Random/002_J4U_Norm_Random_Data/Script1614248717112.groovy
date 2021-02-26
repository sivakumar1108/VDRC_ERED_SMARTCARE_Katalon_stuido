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
Response = WS.sendRequest(findTestObject('004_J4U_Apioffers_automation/02_Random Normal/j4u-Normal Random Data', [('j4u_Apioffer_Url') : GlobalVariable.j4u_Apioffer_Url
            , ('j4u_Apioffer_target_msisdn') : GlobalVariable.j4u_Apioffer_target_msisdn, ('j4u_Apioffer_random_msisdn') : GlobalVariable.j4u_Apioffer_random_msisdn]), 
    FailureHandling.CONTINUE_ON_FAILURE)

offer1 = WS.getElementPropertyValue(Response, 'Offers[0].OfferID', FailureHandling.CONTINUE_ON_FAILURE)

offer2 = WS.getElementPropertyValue(Response, 'Offers[1].OfferID', FailureHandling.CONTINUE_ON_FAILURE)

offer3 = WS.getElementPropertyValue(Response, 'Offers[2].OfferID', FailureHandling.CONTINUE_ON_FAILURE)

def list = ['612135', '612134', '612133', '612132', '612131', '612130', '612129', '612119', '612118', '612117', '612116'
    , '612115', '612113', '612112', '612111', '612114', '612110', '612128', '612127', '612126', '612125', '612124', '612123'
    , '612122', '612121', '612120', '612080', '612079', '612077', '612075', '612073', '612060', '612055', '612054', '612053'
    , '612052', '612049', '612048', '612047', '612046', '612045', '612044', '612043', '612042', '612041', '612040', '612039'
    , '612038', '612037', '612036', '612035', '612021', '612020', '612019', '612018', '612017', '612016', '612015', '612014'
    , '612013', '612012', '612011', '612026', '612025', '612024', '612023', '612022']

println(offer1)

println(offer2)

println(offer3)

assert offer1 in list

assert offer2 in list

assert offer3 in list

WS.comment('Normal Random Data offers flow is working fine.')

