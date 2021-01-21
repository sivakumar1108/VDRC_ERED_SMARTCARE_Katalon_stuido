<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>smartcare-INACTIVEUSER_for test case</name>
   <tag></tag>
   <elementGuidId>f6108d27-f38e-49f6-9e08-35a4a92d20ac</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;\u003csoapenv:Envelope xmlns:soapenv\u003d\&quot;http://schemas.xmlsoap.org/soap/envelope/\&quot; xmlns:v1\u003d\&quot;http://seq.huawei.com/interface/dsi/v1.1\&quot;\u003e\n\u003csoapenv:RequestHeader\u003e\n\t\u003creqTime\u003e2020-12-13 00:04:00\u003c/reqTime\u003e\n\t\u003c/soapenv:RequestHeader\u003e\n \u003csoapenv:RequestBody\u003e\n\u003cUserList\u003e\n \u003cUser\u003e\n \u003cmsisdn\u003e${msisdn}\u003c/msisdn\u003e\n \u003cimsi\u003e630010564977130\u003c/imsi\u003e\n \u003ccellid\u003e6300102F1602\u003c/cellid\u003e\n \u003cstarttime\u003e${starttime}\u003c/starttime\u003e\n \u003clastactivedate\u003e${lastactivedate}\u003c/lastactivedate\u003e\n \u003c/User\u003e                \n\u003c/UserList\u003e\n   \u003c/soapenv:RequestBody\u003e\n\u003c/soapenv:Envelope\u003e&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/plain</value>
   </httpHeaderProperties>
   <katalonVersion>7.8.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${smartcare_url}/SmartCarePlugin/WSDLLoader?group=INACTIVEUSER</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.msisdn</defaultValue>
      <description></description>
      <id>217badff-1110-48c9-a090-c915d3165899</id>
      <masked>false</masked>
      <name>msisdn</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.starttime</defaultValue>
      <description></description>
      <id>568fdac3-81e2-4585-b59b-028ca096cfa8</id>
      <masked>false</masked>
      <name>starttime</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.lastactivedate</defaultValue>
      <description></description>
      <id>08a97666-ba60-4722-adeb-0e40bb598e87</id>
      <masked>false</masked>
      <name>lastactivedate</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.smartcare_url</defaultValue>
      <description></description>
      <id>a5608a02-8553-4c88-884f-5de2c9439c70</id>
      <masked>false</masked>
      <name>smartcare_url</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
